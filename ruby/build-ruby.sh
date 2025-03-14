#! /usr/bin/sh

# cd to location of bash script
cd $(dirname "$0")

# download or update ruby-build
if [ -d "ruby-build" ]; then
  # silence!!!!
  git -C ruby-build pull > /dev/null
else
  git clone https://github.com/rbenv/ruby-build.git
fi

RUBY_CFLAGS="-Og -ggdb" RUBY_CONFIGURE_OPTS="--enable-install-static-library --enable-shared" ruby-build/bin/ruby-build "3.1.6" pfx/