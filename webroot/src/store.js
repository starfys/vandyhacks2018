import Vue from 'vue';
import Vuex from 'vuex';

import api from "@/api";
import router from "@/router";

Vue.use(Vuex);

const authModule = {
  state: {
    user: null,
  },
};

const taskModule = {
  getters: {
    currentlyBuildingTask(state) {
      return state.taskAdd.currentTask !== null;
    }
  },
  state: {
    taskList: {
      tasks: [],
    },
    taskAdd: {
      currentTask: null,
    },
  },
  mutations: {
    setTaskList(state, newList) {
      state.taskList.tasks = [...newList];
    },
    initTaskAdd(state) {
      state.taskAdd.currentTask = {};
    },
    resetTaskAdd(state) {
      state.taskAdd.currentTask = null;
    },
  },
  actions: {
    async updateTaskList({rootState, commit}) {
      const user = rootState.auth.user;
      const data = await api.list(user);
      commit("setTaskList", data);
    },
    async addTask({ rootState, dispatch, commit }, task) {
      const user = rootState.auth.user;
      const returnedTask = await api.tasks.add(user, task);
      await dispatch("updateTaskList");
    },
    beginAddingTask({commit}) {
      router.push("/task/add");
      commit("initTaskAdd");
    },
    async finishAddingTask({state, commit}) {
      const task = state.taskAdd.currentTask;
      await dispatch("addTask", task);
      commit("resetTaskAdd");
    },
  },
};

export default new Vuex.Store({
  modules: {
    tasks: taskModule,
    auth: authModule,
  },
})
