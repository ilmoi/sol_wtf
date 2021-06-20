<template>
  <div class="text-sm">
    <!--profile-->
    <div class="flex items-center">
      <img :src="tweet_object.author.profile_image" class="rounded-full">
      <div class="m-2">
        <div class="flex">
          <div class="font-bold mr-2">{{ tweet_object.author.twitter_name }}</div>
          <div class="text-gray-500 dark:text-gray-400 mr-2">@{{ tweet_object.author.twitter_handle }}</div>
        </div>
        <div class="text-gray-500 dark:text-gray-400">{{ tweet_object.tweet.tweet_created_at }}</div>
      </div>
    </div>

    <!--body-->
    <div class="flex">
      <div class="vertical_line"></div>
      <div>
        <!--text-->
        <div class="text-body">{{ tweet_object.tweet.tweet_text }}</div>
        <!--media-->
        <div v-if="tweet_object.media.length > 0">
          <img v-for="media_object in tweet_object.media" :src="media_object.display_url" class="media"/>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    tweet_object: Object,
  },
}
</script>

<style scoped>
p {
  @apply mr-1;
}

.text-body {
  @apply m-2;
  white-space: pre-wrap;
}

.vertical_line {
  @apply bg-solana-purple dark:bg-solana-green m-4 ml-5;
  width: 1px;
}

.media {
  @apply rounded-lg;
  width: 100%;
}
</style>

