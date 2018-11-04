<template>
  <section>
    <task-card
      :animate="true"
      v-for="task in taskList"
      v-bind:key="task.id"
      v-bind:task="task"
      class="task-item"></task-card>
    <button class="fab" v-on:click="newTask()">
      <v-icon class="square" name="plus" scale="2.3" />
    </button>
  </section>
</template>

<style>
.task-item {
  margin: 5px;
}
.square {
  height: 50px;
  width: 50px;
}
.fab {
  position: absolute;
  bottom: 20px;
  right: 20px;
  padding: 5px;
  box-shadow: 0 3px 6px rgba(0,0,0,0.16), 0 3px 6px rgba(0,0,0,0.23);
  background: none;
  color: var(--primary);
  border: none;
  border-radius: 40px;
}
</style>

<script>
import { mapState, mapActions, } from "vuex";

import TaskCard from "@/components/TaskCard.vue";

export default {
  name: "TaskList",

  data() {
    return {};
  },

  created() {
    this.updateTaskList();
  },

  computed: {
    ...mapState({
      taskList: state => state.tasks.taskList.tasks,
    }),
  },

  components: {
    TaskCard,
  },

  methods: {
    ...mapActions([
      "updateTaskList",
    ]),
    newTask() {
      this.$router.push("/tasks/add");
    },
  },
};
</script>
