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
        if (   ( $File::Find::dir ne $File::Find::name )
            && ( $File::Find::dir !~ /^local\// )
            && ( index( $File::Find::dir, "/node_modules" ) == -1 )
            && ( $_ =~ /\.(md|js|jsx|json|ts|tsx)$/ ) )
        {
            say $File::find::dir;
            system( ( "deno", "fmt", $File::Find::name ) );
        }
    };
    File::Find::find( $fmt_cmd, ( $FindBin::Bin . "/.." ) );
}
