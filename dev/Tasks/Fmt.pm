use 5.38.0;
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
        if (   ( -f $_ )
            && ( index( $_, "/local/" ) != -1 )
            && ( index( $_, "/node_modules/" ) != -1 )
            && ( $_ =~ /\.(md|js|jsx|json|ts|tsx)$/ ) )
        {
            say $File::Find::dir;

            # system( ( "deno", "fmt", $File::Find::name ) );
        }
    };
    File::Find::find( $fmt_cmd, ( $FindBin::Bin . "/.." ) );
}
