<template>
  <section>
    <task-input v-model="origTask" />
    <b-button @click="onSave" variant="primary">Save</b-button>
  </section>
</template>

<script>
import { mapState, mapActions, } from "vuex";

import TaskInput from "@/components/TaskInput.vue";

export default {
  data() {
    return {
      task: {
        name: "",
        description: "",
        due: "",
      }
    };
  },

  computed: {
    ...mapState(["tasks"]),
    taskId() {
      return Number(this.$route.params.id);
    },
    origTask() {
      const found =
        this.tasks.taskList.tasks.find(task=>task.task_id==this.taskId);
      
      return found ? found : {};
    },
  },

  methods: {
    ...mapActions([
      "modifyTask",
    ]),
    onSave() {
      this.modifyTask([this.taskId, {
        name: this.task.name,
        description: this.task.description,
        due: Date.now(this.task.due),
      }]);
    },
  },

  components: {
    TaskInput,
  },
};
</script>
