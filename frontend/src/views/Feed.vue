<template>
  <div>
    <div class="flex flex-col items-center justify-center pt-2" id="top">
      <!--TITLE-->
      <div class="title adj_width">
        <!--basic-->
        <!-- :class="this.$store.state.theme === 'dark' ? 'sol-dark' : 'sol-light' "-->
        <h1 class="solwtf">sol.wtf</h1>
        <div class="flex items-center justify-center">
          <p>wtf happened on</p> <p class="italic text-solana-green dark:text-solana-pink">$SOL</p><p>twitter</p>
          <!--<select class="text-black">-->
            <!--<option>twitter</option>-->
          <!--</select>-->
        </div>
        <div class="flex items-center justify-center">
          <p>over the past </p>
          <select v-model="timeframe" @change="changeType" class="text-black">
            <!--<option>1h</option>-->
            <option>4h</option>
            <option>24h</option>
            <option>48h</option>
            <option>1 week</option>
            <!--<option>1 month</option> (!)FOR NOW REMOVING-->
          </select>
        </div>
        <div class="flex items-center justify-center">
          <p>sorted by </p>
          <select v-model="sort_by" @change="changeType" class="text-black">
            <option>popularity</option>
            <option>retweets</option>
            <option>likes</option>
            <option>replies</option>
            <option>time</option>
          </select>
        </div>

        <!--advanced-->
        <HiddenDetails title="advanced filters">
          <div class="flex items-center justify-center">
            <p>include words: </p>
            <input v-model="include" class="text-black" placeholder="sbf, anatoly" @input="handleInput(include)">
          </div>
          <div class="flex items-center justify-center">
            <p>exclude words: </p>
            <input v-model="exclude" class="text-black" placeholder="vitalik, satoshi" @input="handleInput(exclude)">
          </div>

          <!--<p class="mt-2">include posts from:</p>-->
          <!--<div>-->
          <!--  <input type="checkbox" id="solana" v-model="solana" class="checkbox">-->
          <!--  <label for="solana">solana core team</label>-->
          <!--</div>-->

          <!--<div>-->
          <!--  <input type="checkbox" id="projects" v-model="projects" class="checkbox">-->
          <!--  <label for="projects">projects</label>-->
          <!--</div>-->

          <!--<div>-->
          <!--  <input type="checkbox" id="traders" v-model="traders" class="checkbox">-->
          <!--  <label for="traders">traders</label>-->
          <!--</div>-->

          <!--<div>-->
          <!--  <input type="checkbox" id="hackers" v-model="hackers" class="checkbox">-->
          <!--  <label for="hackers">hackers</label>-->
          <!--</div>-->

          <!--<div>-->
          <!--  <input type="checkbox" id="news" v-model="news" class="checkbox">-->
          <!--  <label for="news">news outlets</label>-->
          <!--</div>-->


          <!--<div class="mt-4">-->
          <!--  <input type="checkbox" id="remember" v-model="remember" class="checkbox">-->
          <!--  <label for="remember">Remember my choices</label>-->
          <!--</div>-->

        </HiddenDetails>
        <!--<button @click="pullStuff">pull</button>-->
      </div>

      <!--FEED-->
      <div>
        <div v-for="t in tweets" :key="t.tweet.tweet_id">
          <div class="flex justify-center">
            <!--LEFT-->
            <Tweet :tweet_object="t" v-if="shouldShow(t)"/>

            <!--RIGHT-->
            <!--<TweedEmbed :id="t.tweet.tweet_id" class="adj_width">loading...</TweedEmbed>-->
          </div>
        </div>
      </div>

    </div>
    <!--must sit at the bottom of the outer most div, or won't work-->
    <infinite-loading @infinite="fetchMoreData" :identifier="infiniteId"></infinite-loading>
  </div>
</template>

<script>
import axios from "axios";
import Tweet from "@/components/Tweet"
import HiddenDetails from "@/components/HiddenDetails";
import {Tweet as TweedEmbed} from 'vue-tweet-embed';
import {fetchSecure} from "@/helpers";

export default {
  components: {
    HiddenDetails,
    Tweet,
    TweedEmbed,
  },
  data() {
    return {
      // feed
      tweets: [],
      page: 1,
      infiniteId: +new Date(),
      // query params
      sort_by: "popularity",
      timeframe: "24h",
      last_tweet_id: "922337", //largest int
      last_metric: "2036854775807", //postgres supports
      // form
      include: "",
      includeArray: [],
      exclude: "",
      excludeArray: [],
      solana: true,
      projects: true,
      traders: true,
      hackers: true,
      news: true,
      remember: false,
      // colors
      theme: localStorage.getItem('theme') || 'light'
    }
  },
  computed: {
    serializedTimeframe() {
      switch (this.timeframe) {
        case "1h":
          return "hour"
        case "4h":
          return "four"
        case "24h":
          return "day"
        case "48h":
          return "twodays"
        case "1 week":
          return "week"
        case "1 month":
          return "month"
      }
    },
    serializedSortBy() {
      switch (this.sort_by) {
        case "popularity":
          return "popularity_count"
        case "retweets":
          return "combined_retweet_count"
        case "likes":
          return "like_count"
        case "replies":
          return "reply_count"
        case "time":
          return "tweet_created_at"
      }
    },
  },
  methods: {
    // toTitleCase(str) {
    //   return str.toLowerCase().split(' ').map(function (word) {
    //     return (word.charAt(0).toUpperCase() + word.slice(1));
    //   }).join(' ');
    // },
    async fetchMoreData($state = null) {
      const data = await fetchSecure("tweets",
          {
            params: {
              sort_by: this.sort_by,
              timeframe: this.serializedTimeframe,
              last_tweet_id: this.last_tweet_id,
              last_metric: this.last_metric,
            }
          }
      )

      if (data.length > 0) {
        this.tweets.push(...data)
        this.page += 1
        this.last_tweet_id = data[data.length-1].tweet.tweet_id
        this.last_metric = data[data.length-1].tweet[this.serializedSortBy]

        // let count = 1
        // data.forEach(t => {
        //   console.log(`${count} pushing new tweet: ${t.tweet.tweet_id}`)
        //   count += 1
        // })

        console.log(`new page is ${this.page}`)
        console.log(`last metric / tweet id: ${this.last_metric} ${this.last_tweet_id}`)
        console.log(this.tweets)

        $state ? $state.loaded() : null
        return true
      } else {
        $state ? $state.complete() : null
        return false
      }
    },
    changeType() {
      this.tweets = []
      this.page = 1
      this.last_tweet_id = "922337" //largest int
      this.last_metric = "2036854775807" //postgres supports
      this.infiniteId += 1
    },
    handleInput(filterStr) {
      const words = filterStr.toLowerCase().split(",")
      let newWords = []
      words.forEach(w => {
        // w = w.replace(",", '');
        w = w.trim()
        w ? newWords.push(w) : null
      })
      if (filterStr === this.include) {
        this.includeArray = newWords
      } else {
        this.excludeArray = newWords
      }
      this.infiniteId += 1
    },
    shouldShow(t) {
      if (this.includeArray.length === 0 && this.excludeArray.length === 0) return true

      const bodyText = String(t.tweet.tweet_text).toLowerCase()
      const name = String(t.author.twitter_name).toLowerCase()
      const handle = String(t.author.twitter_handle).toLowerCase()

      let flag;

      if (this.includeArray.length !== 0) {
        flag = false
      } else {
        flag = true
      }

      this.includeArray.forEach(w => {
        if (bodyText.indexOf(w) > -1) flag = true
        else if (name.indexOf(w) > -1) flag = true
        else if (handle.indexOf(w) > -1) flag = true
      })

      this.excludeArray.forEach(w => {
        if (bodyText.indexOf(w) > -1) flag = false
        else if (name.indexOf(w) > -1) flag = false
        else if (handle.indexOf(w) > -1) flag = false
      })

      return flag
    },
    pullStuff() {
      fetchSecure("pull")
    }
  },
  created() {
    console.log(this.$store.state.count)
  }
}
</script>

<style>
.title {
  @apply bg-solana-purple dark:bg-solana-green
  text-center text-white dark:text-black my-5 p-5 py-10 flex flex-col justify-center items-center;
}

.solwtf {
  @apply fiveh:text-6xl fiveh:text-shadow-sol-lg-l !important;
  @apply text-4xl fourh:text-5xl text-shadow-sol-md-l fourh:text-shadow-sol-md-l text-solana-green dark:text-solana-pink;
  @apply mb-5;
  font-family: 'atari', monospace;
}

.checkbox {
  @apply checked:bg-red-500 mr-1 checked:border-black;
}
</style>