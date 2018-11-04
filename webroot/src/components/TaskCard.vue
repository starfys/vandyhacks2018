<template>
  <b-card class="task-card">
    <div class="shelf">
      <h3 class="title"><b-badge class="shelf-badge">{{task.progress}}%</b-badge> {{task.name}}</h3>
      <b-btn-group>
        <b-btn :href="editTaskUrl">
          <v-icon name="edit" scale="1" />
        </b-btn>
        <b-btn v-if="active" @click="stopWorkOnTask">
          <v-icon name="stop" scale="1" />
        </b-btn>
        <b-btn v-else @click="startWorkOnTask">
          <v-icon name="play" scale="1" />
        </b-btn>
      </b-btn-group>
    </div>
    <b-progress
      :striped="true"
      :animated="active"
      :value="task.progress"
      :max="100"
      class="prog-bar" />
  </b-card>
</template>

<style>
.task-card {
  border: none;
}
.prog-bar {
  margin: 10px 0;
  border-radius: 7px;
}
.title {
  text-align: left;
  margin-bottom: 0;
}
.shelf-badge {
}
.shelf {
  display: flex;
  justify-content: space-between;
  flex-flow: row nowrap;
  align-items: flex-start;
}
</style>

<script>
import { mapActions } from "vuex";

export default {
  name: "TaskCard",

  props: {
    task: Object,
    active: Boolean,
  },

  computed: {
    editTaskUrl() {
      return `/tasks/${this.task.task_id}`;
    },
  },

  methods: {
    ...mapActions([
      "startWork",
    ]),
    startWorkOnTask() {
      //console.log("Stub for startWorkOnTask.");
      this.startWork(this.task.task_id);
    },
    stopWorkOnTask() {
      this.$router.push(`/tasks/${this.task.task_id}/work`);
    },
  },
};
</script>
