<template>
  <div v-html="compiledMarkdown"></div>
</template>

<script setup>
import { computed } from 'vue';
import { marked } from 'marked';
// import DOMPurify from 'dompurify';

const props = defineProps({
  markdown: {
    type: String,
    required: true
  }
});

const compiledMarkdown = computed(() => {
  // return DOMPurify.sanitize(marked(props.markdown));
  return marked(props.markdown);
});
</script>