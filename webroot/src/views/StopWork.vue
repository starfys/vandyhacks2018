<template>
  <section>
    <header class="question-header">
      <!--<b-btn variant="link" class="title-icon">
        <v-icon name="chevron-left" scale="2" />
      </b-btn>-->
      <h2>{{task ? task.name : ""}}</h2>
      <b-btn @click="exit" variant="link" class="title-icon">
        <v-icon name="times" scale="2" />
      </b-btn>
    </header>
    <b-carousel controls
      id="work-carousel"
      img-height="60"
      img-width="100%"
      v-model="carouselIndex"
      :interval="0"
      class="main-carousel">
      <b-carousel-slide img-blank>
        <p class="carousel-text">What's your progress?</p>
        <b-form-input type="number" v-model="newProgress" />
        <b-btn v-on:click="carouselIndex++" class="save-btn">Save</b-btn>
      </b-carousel-slide>

      <b-carousel-slide img-blank>
        <span class="carousel-text">Did you listen to music?</span>
        <b-form-radio-group buttons v-model="boundWorkData.music">
          <b-form-radio :value="true">Yes</b-form-radio>
          <b-form-radio :value="false">No</b-form-radio>
        </b-form-radio-group>
        <br/>
        <b-btn v-on:click="submit('music')" class="save-btn">Save</b-btn>
      </b-carousel-slide>

      <b-carousel-slide img-blank>
        <p class="carousel-text">How many meetings did you have?</p>
        <b-form-input type="number" v-model="boundWorkData.meetings" />
        <b-btn v-on:click="submit('meetings')" class="save-btn">Save</b-btn>
      </b-carousel-slide>

      <b-carousel-slide img-blank>
        <p class="carousel-text">How many breaks did you have?</p>
        <b-form-input type="number" v-model="boundWorkData.breaks" />
        <b-btn v-on:click="submit('breaks')" class="save-btn">Save</b-btn>
      </b-carousel-slide>

      <b-carousel-slide img-blank>
        <p class="carousel-text">How many interruptions did you have?</p>
        <b-form-input type="number" v-model="boundWorkData.interruptions" />
        <b-btn v-on:click="submit('interruptions')" class="save-btn">Save</b-btn>
      </b-carousel-slide>

      <b-carousel-slide img-blank>
        <p class="carousel-text">How noisy was the room?</p>
        <b-form-input type="number" v-model="boundWorkData.noise" />
        <b-btn v-on:click="submit('noise')" class="save-btn">Save</b-btn>
      </b-carousel-slide>
    </b-carousel>

    <b-btn @click="finish">Finish</b-btn>
  </section>
</template>

<style>
.question-header {
  padding: 0 50px;

  display: flex;
  flex-flow: row no-wrap;
  justify-content: space-between;
  align-items: center;
}
.main-carousel {
  color: black;
}
.carousel-item {
  font-size: 20px;
}
.carousel-text {
  color: black;
}
.save-btn {
  margin-top: 10px;
}
</style>

<script>
import { mapGetters, mapActions } from "vuex";

export default {
  name: "StopWork",

  data() {
    return {
      carouselIndex: 0,
      newProgress: "0",
      boundWorkData: {
        meetings: 0,
        breaks: 0,
        interruptions: 0,
        noise: 0,
        music: false,
      },
      cleanWorkData: {},
    };
  },

  computed: {
    ...mapGetters([
      "taskById",
    ]),

    taskId() {
      return this.$route.params.id;
    },

    task() {
      return this.taskById(Number(this.taskId));
    },
  },

  methods: {
    ...mapActions([
      "finishWork",
    ]),
    submit(dataName) {
      this.cleanWorkData[dataName] = this.boundWorkData[dataName];
      this.carouselIndex++;
    },
    exit() {
      this.$router.go(-1);
    },
    finish() {
      console.log("num", Number(this.newProgress));
      this.finishWork([this.taskId, Number(this.newProgress), this.cleanWorkData]);
      this.exit();
    },
  },
};
</script>
