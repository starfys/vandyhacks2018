<template>
  <b-card class="task-card">
    <div class="shelf">
      <h3 class="title"><b-badge class="shelf-badge">{{task.progress}}%</b-badge> {{task.name}}</h3>
      <b-btn-group>
        <b-btn><v-icon name="edit" scale="1" /></b-btn>
        <b-btn><v-icon name="play" scale="1" /></b-btn>
      </b-btn-group>
    </div>
    <b-progress
      :striped="true"
      :animated="animate"
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
export default {
  name: "TaskCard",
  props: {
    task: Object,
    animate: Boolean,
  },
};
</script>
