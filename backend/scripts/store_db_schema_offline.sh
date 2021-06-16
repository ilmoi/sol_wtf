# need to run cargo clean otherwise get this error: https://github.com/launchbadge/sqlx/issues/788
cargo clean
cargo sqlx prepare -- --bin backend