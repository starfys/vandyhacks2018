import axios from "axios";

const api = axios.create({
  baseURL: "/api",
});

export default {
  tasks: {
    async list(user) {
      const res = await api.get(`/task/list/${user.id}`);

      return res.data;
    },
    async add(user, task) {
      const res = await api.post(`/task/add`, task);

      return res.data;
    },
    async delete(user, taskId) {
      const res = await api.post(`/task/remove/${taskId}`);

      return res.data;
    },
    async modify(user, taskId, taskData) {
      const res = await api.post(`/task/modify/${taskId}`, taskData);

      return res.data;
    },
  },

  work: {
    async start(user, taskId) {
      const res = await api.post(`/work/start/${taskId}`);

      return res.data;
    },
    async finish(user, taskId, workData) {
      const res = await api.post(`/work/finish/${taskId}`, workData);

      return res.data;
    },
  },

  users: {
    async add(user, userData) {
      const res = await api.post(`/user/add`);

      return res.data;
    },
    async delete(user, userId) {
      const res = await api.post(`/user/remove/${userId}`);

      return res.data;
    },
    async modify(user, userId, userData) {
      const res = await api.post(`/user/modify/${userId}`, userData);
    },
  },
};
