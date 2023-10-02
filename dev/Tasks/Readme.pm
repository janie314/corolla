use 5.38.0;
use strict;
use warnings;
use FindBin;
use Cwd;

package Tasks::Readme;

use base 'Exporter';
our @EXPORT = qw(compile_readme);

sub compile_readme {
  my $dir = $FindBin::Bin;
  my $file;
  open( $file, '>', "$dir/../README.md" ) or die "couldn't open README file";

  my $helptext = `$dir/../corolla -h`;
  $helptext =~ s/^\s+|\s+$//g;

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

```
$helptext
```
README

  print $file $README;
}

1
