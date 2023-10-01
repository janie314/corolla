use 5.28.0;
use strict;
use warnings;
use FindBin;
use Cwd;

my $cwd = getcwd;
my $file;
open( $file, '>', "$cwd/../README.md" ) or die $!;

my $helptext = system "$cwd/../corolla -h";

my $README = <<"README";
# corolla

a lightweight sqlite web server.

# dependencies

- Perl
  - [Carton](https://metacpan.org/pod/Carton)

# installation

```shell
carton install
./corolla
```

# usage

$helptext
README

print $file, $README;
