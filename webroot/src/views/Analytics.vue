<template>
  <section>
    <header>
      <b-btn to="/analytics/predictions" exact variant="link" class="analytic-button">
        <v-icon name="chart-line" scale="1.7" />
        <span class="link-name">Predictions</span>
      </b-btn>
      <b-btn to="/analytics/productivity" exact variant="link" class="analytic-button">
        <v-icon name="chart-pie" scale="1.7" />
        <span class="link-name">Productivity</span>
      </b-btn>
      <b-btn to="/analytics/meetings" exact variant="link" class="analytic-button">
        <v-icon name="chart-area" scale="1.7" />
        <span class="link-name">Meetings</span>
      </b-btn>
    </header>
    <router-view />
  </section>
</template>

<style lang="scss">
iframe {
  width: 100%;
}
.analytic-button {
  &.active {
    color: black;
  }
}
.link-name {
  padding-left: 12px;
}
</style>
