use 5.28.0;
use strict;
use warnings;
use FindBin;
use lib $FindBin::RealBin;
use Tasks::Fmt;
use Tasks::Readme;

fmt;
compile_readme;
