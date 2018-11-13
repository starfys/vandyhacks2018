<template>
  <b-carousel-slide>
    <slot></slot>
    <div>
      <b-btn @click="onSubmit">Submit</b-btn>
    </div>
  </b-carousel-slide>
</template>

<script>
export default {
  name: "StopWorkQuestion",

  methods: {
    onSubmit() {
      this.$emit("submit");
    },
  },
};
</script>
