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
      name: "",
      description: "",
      due: "",
    };
  },
  methods: {
    ...mapActions([
      "addTask",
    ]),
    finalize() {
      this.addTask({
        name: this.name,
        description: this.description,
        due: Date.now(this.due),
      });
    },
  },

  components: {
    TaskInput,
  },
};
</script>
