use 5.28.0;
use strict;
use warnings;
use FindBin;

my $helptext = ``

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

say $README;
