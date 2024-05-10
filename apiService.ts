import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_BACKEND_URL;

class ApiService {
  async authenticateUser(email: string, password: string): Promise<any> {
    try {
      const response = await axios.post(`${API_BASE_URL}/auth/login`, { email, password });
      localStorage.setItem('token', response.data.token);
      return response.data;
    } catch (error) {
      console.error('Error authenticating user:', error);
      throw error;
    }
  }

  async getProjects(): Promise<any[]> {
    try {
      const token = localStorage.getItem('token');
      const response = await axios.get(`${API_BASE_URL}/projects`, {
        headers: { Authorization: `Bearer ${token}` }
      });
      return response.data.projects;
    } catch (error) {
      console.error('Error fetching projects:', error);
      throw error;
    }
  }

  async submitTimeEntry(projectId: string, hours: number, date: Date): Promise<any> {
    try {
      const token = localStorage.getItem('token');
      const response = await axios.post(`${API_BASE_URL}/time-entries`,
        {
          projectId,
          hours,
          date
        },
        {
          headers: { Authorization: `Bearer ${token}` }
        }
      );
      return response.data;
    } catch (error) {
      console.error('Error submitting time entry:', error);
      throw error;
    }
  }

  logout(): void {
    localStorage.removeItem('token');
  }
}

export const apiService = new ApiService();