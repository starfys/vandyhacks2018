<template>
  <div id="app">
    <div id="nav">
      <h1>Taskful</h1>
      <span class="link-holder">
        <b-btn to="/tasks" exact variant="link" class="title-icon">
          <v-icon name="tasks" scale="1.7" />
          <span class="link-name">Tasks</span>
        </b-btn>
        <b-btn to="/analytics" exact variant="link" class="title-icon">
          <v-icon name="chart-bar" scale="1.7" />
          <span class="link-name">Analytics</span>
        </b-btn>
      </span>
    </div>
    <router-view/>
  </div>
</template>

<style lang="scss">
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
#nav {
  display: flex;
  flex-flow: row no-wrap;
  justify-content: space-between;
  align-items: center;
  padding: 30px;

  .link-holder {
    display: flex;
    flex-flow: row no-wrap;
  }

  .title-icon {
    display: flex;
    flex-flow: row no-wrap;
    justify-content: space-between;
    align-items: center;
  
    text-decoration: none;
    font-weight: bold;
    color: var(--primary);

    //TODO: fix bug where `.active` applies to sub-routes as well.
    // Need to find alternative class. `<router-link/>` has some sort of
    // "exact match" class.
    &.active {
      color: #2c3e50;
    }

    .link-name {
      padding-left: 12px;
    }
  }
}
</style>
