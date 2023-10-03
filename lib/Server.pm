use strict;
use warnings;
use 5.38.0;
use FindBin;
use lib "$FindBin::Bin/../local/lib/perl5";
use HTTP::Server::Simple::CGI;
use HTTP::Response;

package Server;
use base qw(HTTP::Server::Simple::CGI);

my %dispatch = ();

sub net_server { 'Net::Server'; }

sub handle_request {
    my ( $self, $cgi ) = @_;

    my $path    = $cgi->path_info();
    my $handler = $dispatch{$path};

    if (1) {
        print HTTP::Response->new( 200, "joemoe" )->decoded_content;
        $handler->($cgi);

    }
}

1;
