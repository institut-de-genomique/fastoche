# fastoche

fastoche is a rdbioseq project

A new version of the environment module associated with this project is deployed upon every commit on master, except if the commit message starts with [NO-DEPLOY]. The default version increment is patch. To ship a new minor or major version of the module, start your commit message with [MAJOR] or [MINOR] respectively.

It is recommanded that you setup tests for fastoche in either or both a `tests` folder or in tests sub-modules depending on their nature (respectively integration- and unit-tests): they will be executed by `cargo test` everytime you push to GitLab, and the deployment stage will not be triggered unless they are successful

This crate can be installed using `cargo install`.
