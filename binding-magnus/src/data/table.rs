use magnus::{Class, Module, Object, RString, TryConvert, Value, function, method};
use std::cell::Cell;

use crate::{AsKey, arenas};

#[derive(Default)]
#[magnus::wrap(class = "Table", size, free_immediately)]
pub struct Table(pub Cell<rgss::TableKey>);

impl From<rgss::TableKey> for Table {
    fn from(value: rgss::TableKey) -> Self {
        Self(Cell::new(value))
    }
}

// removes the Table from arenas.
// this should only happen once, and should only happen when a Table is no longer used!
impl Drop for Table {
    fn drop(&mut self) {
        let mut arenas = crate::arenas::get().write();
        if arenas.tables.remove(self.0.get()).is_none() {
            log::warn!(
                "Table {:p}:{:?} was drop'd twice!",
                self as *mut _,
                self.as_key()
            )
        }
    }
}

impl AsKey for Table {
    type Key = rgss::TableKey;

    fn as_key(&self) -> Self::Key {
        self.0.get()
    }
}

impl Table {
    fn initialize(&self, args: &[Value]) -> magnus::error::Result<()> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;
        let (xsize,) = args.required;
        let (ysize, zsize) = args.optional;

        let mut arenas = arenas::get().write();
        let table = rgss::Table::new(xsize, ysize.unwrap_or(0), zsize.unwrap_or(0));
        let table_key = arenas.tables.insert(table);

        self.0.set(table_key);

        Ok(())
    }

    fn deserialize(bytes: RString) -> Table {
        let mut arenas = arenas::get().write();
        // We don't hold onto the slice long enough for ruby to do anything with it.
        let table = unsafe {
            let bytes = bytes.as_slice();

            let u32_slice: &[u32] = bytemuck::cast_slice(bytes);

            let [_, xsize, ysize, zsize, len, data @ ..] = u32_slice else {
                todo!()
            };
            let data = bytemuck::cast_slice(data).to_vec();
            assert_eq!(*len as usize, data.len());

            rgss::Table::new_data(*xsize as usize, *ysize as usize, *zsize as usize, data)
        };
        let table_key = arenas.tables.insert(table);
        Self::from(table_key)
    }

    fn serialize(table: &Table, _: i32) -> RString {
        let arenas = arenas::get().read();
        let table = &arenas[table.as_key()];
        // FIXME calculate capacity
        let string = RString::buf_new(0);

        let size = 1 + (table.ysize() > 0) as u32 + (table.zsize() > 0) as u32;
        let header = [
            size,
            table.xsize() as u32,
            table.ysize() as u32,
            table.zsize() as u32,
            table.len() as u32,
        ];

        string.cat(bytemuck::bytes_of(&header));
        string.cat(bytemuck::cast_slice(table.data()));

        string
    }

    fn index(&self, args: &[Value]) -> magnus::error::Result<Option<i16>> {
        magnus::scan_args::check_arity(args.len(), 1..=4)?;
        let index = match *args {
            [rb_x] => (TryConvert::try_convert(rb_x)?, 0, 0),
            [rb_x, rb_y] => (
                TryConvert::try_convert(rb_x)?,
                TryConvert::try_convert(rb_y)?,
                0,
            ),
            [rb_x, rb_y, rb_z] => (
                TryConvert::try_convert(rb_x)?,
                TryConvert::try_convert(rb_y)?,
                TryConvert::try_convert(rb_z)?,
            ),
            _ => unreachable!(),
        };

        let arenas = arenas::get().read();
        Ok(arenas[self.as_key()].get(index).copied())
    }

    fn index_set(&self, args: &[Value]) -> magnus::error::Result<()> {
        magnus::scan_args::check_arity(args.len(), 2..=5)?;
        let index = match *args {
            [rb_x, _] => (TryConvert::try_convert(rb_x)?, 0, 0),
            [rb_x, rb_y, _] => (
                TryConvert::try_convert(rb_x)?,
                TryConvert::try_convert(rb_y)?,
                0,
            ),
            [rb_x, rb_y, rb_z, _] => (
                TryConvert::try_convert(rb_x)?,
                TryConvert::try_convert(rb_y)?,
                TryConvert::try_convert(rb_z)?,
            ),
            _ => unreachable!(),
        };
        let value = args.last().copied().map(TryConvert::try_convert).unwrap()?;

        let mut arenas = arenas::get().write();
        if let Some(table_val) = arenas[self.as_key()].get_mut(index) {
            *table_val = value;
        }
        Ok(())
    }
}

pub fn bind(ruby: &magnus::Ruby) -> magnus::error::Result<()> {
    let class = ruby.define_class("Table", ruby.class_object())?;
    class.define_alloc_func::<Table>();
    class.define_method("initialize", method!(Table::initialize, -1))?;
    class.define_singleton_method("_load", function!(Table::deserialize, 1))?;
    class.define_method("_dump", method!(Table::serialize, 1))?;

    class.define_method("[]", method!(Table::index, -1))?;
    class.define_method("[]=", method!(Table::index_set, -1))?;

    Ok(())
}
