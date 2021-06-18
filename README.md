# Project goals

1. Learn rust by building something remotely real
2. Build something useful for the $SOL community
3. OSS the code for future rust learners

# Project description

sol.wtf is a simple resource for anyone short on time to quickly get up to date on the $SOL
ecosystem. It aggregates top posts from Twitter / Reddit / other
[Solana-relevant platfroms](https://solana.com/community). Currently only Twitter is implemented.

What's the difference with eg just using Twitter? Twitter wants you to spend
as MUCH time on their website as possible and so even if you follow the right
people your timeline will be optimized in a way to waste your time. 

sol.wtf is the opposite of that. We want you to spend as LITTLE time on the
website as possible. So we surface the top posts first and you can keep
scrolling for as much (or little) as you want. From personal experirence checking the
website once a day for 2-3min is sufficient to stay in the loop.

# Implementation

The stack is Rust + Vue2 + AWS/Terraform/github actions.

A few decisions I made along the way:
- Scheduled tweet pull occurs as part of main runtime. I decided the operation wasn't heavy enough to involve redis / build a job queue.
- Tweets are pulled every hour. 5 last tweets are pulled for each timeline, without regard for whether they've been pulled previously. This is to update metrics like quote count.
- Default ranking is by popularity (retweet/quote count + like count + comment count).
- Twitter's rate limits are a bitch, keep that in mind. You only get 500k tweets/mo and 900 or 1500 api calls (depending on endpoint) per 15min.
- I had to rebuild twitter's formatting on the front-end because their oembed-js library is slow as fuck.


# Development
- Git clone the repo
- Create a `backend/secrets/twitter.yml` file and format it like so:
```yaml
twitter:
  bearer_token: XXX
```
- Create a `terraform/terraform.tfvars` file and format it like so:
```shell
db_username = "postgres"
db_password = "your_db_pw"
```
- Add the below 2 lines to your `.zshrc` or equivalent:
```shell
export AWS_ACCESS_KEY_ID=xxx
export AWS_SECRET_ACCESS_KEY=xxx
```
- Edit any other config you might want to in `backend/config`

To launch locally:
- `cd` into frontend and do `yarn` then `yarn serve`
- `cd` into backend and do `RUST_LOG=<level> cargo run`, where you replace <level> with debug/info/error etc

To launch in prod:
- from the main dir do `docker-compose -f terraform/docker-compose.TERRA.yml run --rm terraform apply`

# Improvements / future work
- Add other news sources - reddit / discord / more stuff from solana [here](https://solana.com/community)
- Add an option to only show tweets the user hasn't seen already in the last 24h
- Twitter provides the location of each tweet in the API... print on map? Dunno why
- Have a stats page with some charts with number of tweets / posts / etc
- Have a ranking of top posters
- Have a way for the community to vote on what accounts should be followed
- Make the search algo smarter / more efficient
- Add a subscribe feature so you can receive X top posts/tweets into your inbox daily or however often you want
- Fix tweet display on the frontend ([not everything's perfect](https://www.notion.so/ilmoi/better-tweet-display-dad2f209dd154cb1802e01fe5ba7c297))

# Rust resources

I used the following resources (in the below order) to teach myself enough rust to build
this:
1. [Udemy - Rust crash course](https://www.udemy.com/course/ultimate-rust-crash-course/) (3hr). Awesome because short and won't get you bored. Perfect to get started.
2. ["The book"](https://doc.rust-lang.org/stable/book/) / [Rustlings exercises](https://github.com/rust-lang/rustlings/tree/main/exercises) that go with it. Awesome because comprehensive. A bit of a grind but have to do it.
3. [Luca Palmieri's zero2prod book](https://www.zero2prod.com/index.html?country=Latvia&discount_code=EEU60). Awesome because covers rust web servers in great detail - and makes you a better engineer as a whole along the way. I honestly can't recommend this enough (Luca is not paying me).
4. Blood, sweat and tears. Some red bulls for long nights.

# Contributions

Ape in. Any questions, find me here -> iljamoi@pm.me.

# License

MIT 
