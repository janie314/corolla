use strict;
use warnings;
use 5.38.0;
use FindBin;
use lib "$FindBin::Bin/../local/lib/perl5";
use HTTP::Server::Simple::CGI;

package Server;
use base qw(HTTP::Server::Simple::CGI);

my %dispatch = ();

sub handle_request {
    my $self = shift;
    my $cgi  = shift;

    my $path    = $cgi->path_info();
    my $handler = $dispatch{$path};

    if ( ref($handler) eq "CODE" ) {
        print "HTTP/1.0 200 OK\r\n";
        $handler->($cgi);

    }
    else {
        print "HTTP/1.0 404 Not found\r\n";
        print $cgi->header,
          $cgi->start_html('Not found'),
          $cgi->h1('Not found'),
          $cgi->end_html;
    }
}

1;
