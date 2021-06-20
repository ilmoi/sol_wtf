<template>
  <div class="flex flex-col items-center justify-center pt-2">
    <Box>
      <h3>WTF is this sol.wtf thing?</h3>
      <p>A simple news aggregator for the <a href="https://solana.com" target="_blank">Solana</a> ($SOL) ecosystem.</p>
      <p>Currently only twitter is supported - later potentially reddit, discord and
        <a href="https://solana.com/community" target="_blank">other platforms</a> will be added.</p>
      <br/>
      <h3>Why does it need to exist?</h3>
      <p>I created sol.wtf because I personally wanted something like this:</p>
      <ul class="ml-3 list-inside list-items">
        <li>> A feed of $SOL-only posts ☀️</li>
        <li>> Ordered by popularity, not twitter's random algo 🔀</li>
        <li>> That I can check once/day and be done with in 2-3min 🕐</li>
      </ul>
      <p>Basically, I created sol.wtf because I have no fucking time to be checking twitter all the time, but I want to know what's going on in $SOL (because I'm an ape and a bag-holder, of course).</p>
      <p>Maybe this describes you as well, in which case you're welcome.</p>
      <br/>
      <h3>So you basically re-built twitter?</h3>
      <p>Sort of. Sounds useful right hhh? There's actually another reason I did this - to teach myself Rust. 🦀</p>
        <p>sol.wtf is written entirely in Rust on the backend. What's more, all the code is OSS and is freely available for you apes
        <a href="https://github.com/ilmoi/sol_wtf" target="_blank">here</a>. Go bananas and devour it.</p>
      <p>If you get any questions along the way - reach out
        <a href="mailto:iljamoi@pm.me" target="_blank">iljamoi@pm.me</a>. Always happy to help a fellow ape.</p>
      <p>And feel free to share some love❤️ by <router-link to="tip">sending me</router-link> a lil' $COPE or $SOL or whatever tip:)</p>
    </Box>
    <Box>
      <h3>If you still got questions</h3>
      <div>
        <HiddenDetails title="How does your feed algorithm work?">
          <p>A twitter account I created, @soldotwtf, follows some 100-200 quality $SOL accounts. The algorithm then goes into their timelines and pulls down the recent tweets.</p>
          <p>Note that retweets are immediately converted into normal tweets, which is why sometimes you'll see tweets from non-followed accounts on the feed (but they've actually been retweeted by followed ones, so are still relevant).</p>
          <p>Another thing to note: typically the first 10-20 posts are pretty general crypto posts. That's because they get a lot of likes/retweets and always end up at the top of the feed.</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="How is popularity calculated?">
          <p>Popularity = retweet/quote count + like count + reply count.</p>
          <p>Follow?</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="How often does the feed update?">
          <p>Once an hour. More often is hard because twitter only lets you pull 500,000 tweets/month, and you have to pull a minimum of 5 per user timeline per call. So you only get 100k timeline calls/month.</p>
          <p>There are optimization you can make but at this point I cba to work on them. Feel free to pick up
            <a href="https://github.com/ilmoi/sol_wtf" target="_blank">where I left</a>.</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="Will you keep working on this?">
          <p>Dunno. I mainly did it to learn Rust. Maybe if it's useful for the community.</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="What if I want to fork the code / contribute?">
          <p>Do it. Repo's <a href="https://github.com/ilmoi/sol_wtf" target="_blank">here</a>.</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="What if I found a bug / have an idea?">
          <p>Fix it / built it. Repo's <a href="https://github.com/ilmoi/sol_wtf" target="_blank">here</a>.</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="How can this project be improved?">
          <p>There's a list in the readme of the repo <a href="https://github.com/ilmoi/sol_wtf" target="_blank">here</a>.</p>
        </HiddenDetails>
      </div>

      <div>
        <HiddenDetails title="Who's hosting this?">
          <p>Me. On AWS. But all terraform code is part of the repo so you can get it up and running on your AWS account in <5 mins.</p>
        </HiddenDetails>
      </div>
      
      <div>
        <HiddenDetails title="And who are you exactly?">
          <p>Fellow ape. See some links to my online persona <a href="http://ilmo.hns.to/" target="_blank">here</a></p>
        </HiddenDetails>
      </div>

    </Box>
  </div>
</template>

<script>
import Box from "@/components/Box";
import HiddenDetails from "@/components/HiddenDetails";
export default {
  components: {HiddenDetails, Box}
}
</script>

<style scoped>

h3 {
  @apply text-xl font-bold m-1
}

a {
  @apply text-solana-blue;
}

</style>