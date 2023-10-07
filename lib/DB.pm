use strict;
use warnings;
use 5.38.0;
use FindBin;
use lib "$FindBin::Bin/../local/lib/perl5";
use Future::Mutex;

package DB;

sub new {
    my ( $class, $dbfile ) = @_;
    $dbfile =~ s~^:~./:~;
    my $db = DBI->connect("dbi:SQLite:dbname=$dbfile") or die $DBI::errstr;
    $db->prepare("PRAGMA journal_mode;")->execute()    or die $DBI::errstr;
    return bless { "db" => $db }, $class;
}

our @EXPORT_OK = qw(read_query);

1;
