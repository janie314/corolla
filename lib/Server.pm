use strict;
use warnings;
use 5.38.0;
use FindBin;
use lib "$FindBin::Bin/../local/lib/perl5";
use Carp;
use Mojolicious::Lite;

sub run_server {
    my ( $port, $root ) = @_;
    $root =~ s~^([^/])~/$1~;
    $root =~ s~/$~~;
    get $root. '/' => sub {
        my ($c) = @_;
        $c->render( text => 'Hello World!' );
    };
    app->secrets( ['I suppose this is important.'] );
    app->start( 'daemon', '-l', "http://*:$port" );
}
