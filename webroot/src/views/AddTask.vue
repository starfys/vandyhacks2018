<template>
  <section>
    <task-input v-model="task" />
    <b-button @click="finalize" variant="primary">Submit</b-button>
  </section>
</template>

<script>
import { mapActions } from "vuex";

import TaskInput from "@/components/TaskInput";

export default {
  data() {
    return {
      task: {
        name: "",
        description: "",
        due: "",
      },
    };
  },
  methods: {
    ...mapActions([
      "addTask",
    ]),
    finalize() {
      this.addTask({
        name: this.task.name,
        description: this.task.description,
        due: Date.now(this.task.due),
      });
      this.$router.push("/tasks");
    },
  },

  components: {
    TaskInput,
  },
};
</script>
