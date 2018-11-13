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
      const res = await api.post(`/user/${user.id}/task`, {
        owner_id: user.id,
        created: Date.now() * 1000,
        importance: 0,
        due: task.due * 1000,
        name: task.name,
        description: task.description,
      });

      return res.data;
    }, async delete(user, taskId) {
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
    async finish(user, taskId, updatedProgress, questionAnswers) {
      console.log(updatedProgress);
      const reqBody = {
          finished: false,
          progress: Number(updatedProgress)/100,
          noise: Number(questionAnswers.noise),
          interruptions: Number(questionAnswers.interruptions),
          meetings: Number(questionAnswers.meetings),
          music: questionAnswers.music,
      };

      console.log(reqBody);
        
      const res = await api.post(
        `/user/${user.id}/task/${taskId}/finish`, reqBody);

      return res.data;
    },
  },

  users: {
    async add() {
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
