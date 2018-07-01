# WaltonProxy
Walton Proxy written in Rust
Compiled with newest nightly Rust version.

## How to use WaltonProxy:

###USAGE:
    WaltonProxy [FLAGS] [OPTIONS]

###FLAGS: <br>
    -d               Set debug log
    -h, --help       Prints help information
    -V, --version    Prints version information

###OPTIONS:
    -c <client>        Set client address of miner: Example: '127.0.0.1'  ->Default:'127.0.0.1'
    -g <gpu>           Set amount of gpu's                                ->Default: 1
    -p <port>          Set starting port of Ming_run.exe files            ->Default: 12126
    -s <server>        Set server address: Example: '127.0.0.1'           ->Default:'127.0.0.1'
###EXAMPLE:
    WaltonProxy.exe -g 2 -p 12126 -c "127.0.0.1" -s "127.0.0.1" -d
    -> Starts WaltonProxy for 2 GPU's that start on port 12126. Server Address is 127.0.0.1 
       and Client Address too. Debug LOG enabled.