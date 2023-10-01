use 5.28.0;
use strict;
use warnings;
use FindBin;
use File::Find;
use Cwd;

package Tasks::Fmt;

use base 'Exporter';
our @EXPORT = qw(fmt);

sub fmt {
    my $fmt_cmd = sub {
        say $File::Find::dir;
    };
    say $FindBin::Bin;
    File::Find::find( $fmt_cmd, ($FindBin::Bin) );
}
