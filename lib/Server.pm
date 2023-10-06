use strict;
use warnings;
use 5.38.0;
use FindBin;
use lib "$FindBin::Bin/../local/lib/perl5";

use Mojolicious::Lite;

sub run_server {
    get '/' => sub {
        my ($c) = @_;
        $c->render( text => 'Hello World!' );
    };

    app->start( 'daemon', '-l', 'http://*:12345' );
}
