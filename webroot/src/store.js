import Vue from 'vue';
import Vuex from 'vuex';

import api from "@/api";
//import router from "@/router";

Vue.use(Vuex);

const authModule = {
  state: {
    user: {
      id: 0, // TEMP variable
    },
  },

  mutations: {
    setUser(state, user) {
      state.user = user;
    },
  },

  actions: {
    async signUp({commit}) {
      const user = await api.users.add();
      commit("setUser", user);
    },
  },
};

const taskModule = {
  state: {
    taskList: {
      tasks: [],
    },
  },

  getters: {
    taskById: state => id => {
      const found = state.taskList.tasks.find(task=>task.task_id==id);
      console.log("found",found);
    },
  },

  mutations: {
    setTaskList(state, newList) {
      state.taskList.tasks = [...newList];
    },
    /*updateTask(state, taskId, newTask) {
      const tasks = state.taskList.tasks;
      const index = tasks.findIndex(({id})=>id === taskId);
      state.taskList.tasks[index] = newTask;
    },*/
  },

  actions: {
    async updateTaskList({rootState, commit}) {
      const user = rootState.auth.user;
      const data = await api.tasks.list(user);
      commit("setTaskList", data);
    },
    // Add a task to the backend
    async addTask({ rootState, dispatch }, task) {
      const user = rootState.auth.user;
      const data = await api.tasks.add(user, task);
      await dispatch("updateTaskList");
      return data;
    },
    async modifyTask({ rootState, dispatch}, taskId, task) {
      const user = rootState.auth.user;
      const data = await api.tasks.modify(user, taskId, task);
      await dispatch("updateTaskList");
      return data;
    },
  },
};

const workModule = {
  /*state: {
    inProgressTaskId: "",
  },

  getters: {
    isTaskActive: state => state.inProgressTaskId != "",
  },*/
  
  actions: {
    async startWork({ rootState, dispatch}, taskId) {
      const user = rootState.auth.user;
      const data = await api.work.start(user, taskId);
      await dispatch("updateTaskList");
      return data;
    },
    async finishWork({ rootState, dispatch}, taskId, workData) {
      const user = rootState.auth.user;
      const data = await api.work.finish(user, taskId, workData);
      await dispatch("updateTaskList");
      return data;
    },
  },
};

const store = new Vuex.Store({
  modules: {
    tasks: taskModule,
    auth: authModule,
    work: workModule,
  },
});

// initial setup of store:
store.dispatch("updateTaskList");

export default store;
