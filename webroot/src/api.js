import axios from "axios";

const api = axios.create({
  baseURL: "/api",
});

export default {
  tasks: {
    async list(user) {
      const res = await api.get(`/user/${user.id}/task`);

      return res.data;
    },
    async add(user, task) {
      const res = await api.post(`/user/${user.id}/task`, task);

      return res.data;
    },
    async delete(user, taskId) {
      const res = await api.delete(`/user/${user.id}/task/${taskId}`);

      return res.data;
    },
    async modify(user, taskId, taskData) {
      const res = await api.put(`/user/${user.id}/task/${taskId}`, taskData);

      return res.data;
    },
  },

  work: {
    async start(user, taskId) {
      const res = await api.post(`/user/${user.id}/task/${taskId}/start`);

      return res.data;
    },
    async finish(user, taskId, workData) {
      const res = await api.post(
        `/user/${user.id}/task/${taskId}/finish`, workData);

      return res.data;
    },
  },

  users: {
    async add(user, userData) {
      const res = await api.post(`/user`);

      return res.data;
    },
    async delete(user, userId) {
      const res = await api.delete(`/user/${userId}`);

      return res.data;
    },
    async modify(user, userId, userData) {
      const res = await api.put(`/user/${userId}`, userData);
    },
  },
};
