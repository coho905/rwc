# rwc
rust implementation of the wc bash command

## Installation
+ To create binaries
  '''bash
  cargo build --release
  '''
+ To add it to the PATH (so that you can run rwc instead of wc), navigate to the rwc home directory and run:
  '''bash
    echo "export PATH=\"\$PATH:\$PWD/target/release\"" >> ~/.zshrc
    source ~/.zshrc
  '''
  NOTE: If you are using a bash shell then add it to bashrc and source bashrc instead of zshrc. This applies for any other shell.

  ## Features to be added:
  + Print out the summed lines, words, bytes from the request.
  + Handle errors in one file without exiting before running others
  + Increase efficiency
  + Anything else
 
  Feel free to make a pull request.

  MIT License, Copyright (c) Colin Wolfe 2024
