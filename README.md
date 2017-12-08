# secret-santa
This program is for those of us who run Secret Santa type events, but want to make it more complex in an attempt to appease more of our friends. In other words, it's for fools like me! That's why I wrote it at 3AM!

In essence, secret-santa builds a directed graph of the people in your Secret Santa network and then finds a Hamilton circuit through the graph. If you specify the '-f / --force' option, it will randomly add edges to the graph until there is a Hamilton circuit. I wouldn't recommend using this setting, since it's terrible.

# Operation
Make sure you have the credentials for an SMTP account. For now, it works using the default settings of lettre, which is TLS over port 587. This works fine for GMail and other GSuite mail applications. For GMail, you will have to make sure you enable less secure apps to log in to your account for this to work properly.

secret-santa is a command line program written in Rust. If you don't have Rust installed, install it by following the instructions on their website (https://www.rust-lang.org). Clone the repository, then run `cargo build` in the root of this directory. Cargo will probably take some time to download and build the required libraries before building the executable itself. Upon completion of the build, you are ready to do some Secret Santa-ing!

First, you must describe the graph of your Secret Santa network. This will be your input file. Format this file as follows:
- The first line is the number of nodes N in your graph, e.g. the number of people who are Secret Santa-ing.
- The next N lines are the nodes of your graph. On each line, write some handle for each person and their email address. Fields are whitespace-delimited. The order of the nodes does not matter.
- The final lines are the edges of your graph. Each line describes one edge. On each line, write two handles for defined nodes. The first handle is the source and the second handle is the target. Order does not matter.

Next, you must provide your credentials for your email account. This isn't terribly secure on your client computer, but it does use TLS so at least nobody will be hacking into your account. If you're worried, you can create a new email account to run it, like I did. This file will have the following contents, each on their own line:
- An SMTP server address
- The email address you are sending from
- The username to log in on the specified SMTP server and email address
- The password to log in on the specified SMTP server and email address

Take these two files, INPUT and CREDENTIALS, and provide them as arguments to secret-santa.

Running secret-santa is as simple as: `cargo run -- INPUT CREDENTIALS`, or if you want to force completion, `cargo run -- INPUT CREDENTIALS --force`. It will give you just enough information so that you know what's going on, but not enough so that you can figure out who has who.
