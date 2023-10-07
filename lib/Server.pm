use strict;
use warnings;
use 5.38.0;
use FindBin;
use lib "$FindBin::Bin/../local/lib/perl5";
use Carp;
use Mojolicious::Lite;
use Future::Mutex;

sub open_db_conn {
    my ($dbfile) = @_;
    $dbfile =~ s~^:~./:~;
    my $db = DBI->connect("dbi:SQLite:dbname=$dbfile") or die $DBI::errstr;
}

sub run_server {

    # process method arguments
    my ( $port, $root, $dbpath ) = @_;
    $root =~ s~^([^/])~/$1~;
    $root =~ s~/$~~;

    # set up endpoints
    get $root. '/read' => sub {
        my ($c)        = @_;
        my $query_name = $c->param('query') || "";
        my $args       = $c->param('args')  || "";
        $c->render( text => "$query_name\n$args" );
    };

    # set $root . '/write' => sub {
    #     my ($c) = @_;
    # };
    app->secrets( ['I suppose this is important.'] );

    # database connection
    helper db => sub { state $db = open_db_conn $dbpath; };

    # init DB
    app->db->do("create table if not exists t (x int);");

    # main execution
    app->start( 'daemon', '-l', "http://*:$port" );
}
