<template>
  <a :href="tweet_object.tweet.tweet_url" target="_blank" class="outer">
    <!--reply-->
    <div v-if="tweet_object.reply_to">
      <RepliedToTweet :tweet_object="tweet_object.reply_to"/>
    </div>

    <!--profile-->
    <div class="flex items-center">
      <img :src="tweet_object.author.profile_image" class="rounded-full">
      <div class="m-2">
        <div class="font-bold">{{ tweet_object.author.twitter_name }}</div>
        <div class="text-gray-500">@{{ tweet_object.author.twitter_handle }}</div>
      </div>
    </div>

    <!-- text -->
    <!--:class="tweet_object.tweet.from_user_timeline ? 'bg-red-100' : 'bg-green-100'"-->
    <div class="text-body">{{ tweet_object.tweet.tweet_text }}</div>

    <!--media-->
    <div v-if="tweet_object.media.length > 0">
      <img v-for="media_object in tweet_object.media" :src="media_object.display_url" class="media"/>
    </div>

    <!--quote-->
    <div v-if="tweet_object.quote_of">
      <QuoteTweet :tweet_object="tweet_object.quote_of"/>
    </div>

    <!--time-->
    <div class="text-gray-500 m-2">{{ tweet_object.tweet.tweet_created_at }}</div>

    <!-- likes -->
    <div>
      <div class="line"></div>
      <div class="flex flex-row">
        <p>❤️</p>
        <p>{{ tweet_object.tweet.like_count }}</p>
        <p>🔁</p>
        <p>{{ tweet_object.tweet.total_retweet_count }}</p>
        <p>💬</p>
        <p>{{ tweet_object.tweet.reply_count }}</p>
      </div>
    </div>
  </a>

</template>

<script>
import QuoteTweet from "@/components/QuoteTweet";
import RepliedToTweet from "@/components/RepliedToTweet";
export default {
  components: {
    RepliedToTweet,
    QuoteTweet,
  },
  props: {
    tweet_object: Object,
  },
}
</script>

<style scoped>
.outer {
  @apply border-gray-200 border-solid border p-2 m-2 rounded;
  width: 550px;
}

p {
  @apply mr-1;
}

.line {
  @apply bg-gray-200;
  height: 1px;
}

.outer:hover {
  @apply bg-gray-50;
}

.text-body {
  @apply m-2;
  white-space: pre-wrap;
}

.media {
  @apply rounded;
  width: 100%;
}
</style>

