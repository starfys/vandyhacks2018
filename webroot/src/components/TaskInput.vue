<template>
  <section class="task-input-component">
    <b-form @submit.prevent="onSave">
      <b-form-input
        v-model="value.name"
        type="text"
        placeholder="Name"
        class="name-input"
        ></b-form-input>

      <b-form-textarea
        v-model="value.description"
        placeholder="Description"
        :rows="3"
        ></b-form-textarea>

      <datetime
        type="datetime"
        v-model="value.due"
        ></datetime>
    </b-form>
  </section>
</template>

<style>
.edit-task-component {

}
.name-input {
  text-align: center;
}
</style>

<script>
export default {
  name: "EditInput",

  data() {
    return {
      name: "",
      description: "",
      due: "",
    };
  },

  props: {
    // The task
    value: Object,
  },

/*  created() {
    console.log(this.value);
    this.name = this.value.name;
    this.description = this.value.description;
    this.due = this.value.due;
  },*/

  methods: {
    onSave() {
      this.$emit("input", {
        name: this.name,
        description: this.description,
        due: this.due,
      });
    },
  },
};
</script>
