import axios, { AxiosError } from 'axios';

const API_BASE_URL = process.env.REACT_APP_BACKEND_URL;

interface LoginResponse {
  token: string;
}

interface Project {
}

interface TimeEntryResponse {
}

class ApiService {
  async authenticateUser(email: string, password: string): Promise<LoginResponse> {
    try {
      const response = await axios.post<LoginResponse>(`${API_BASE_URL}/auth/login`, { email, password });
      localStorage.setItem('token', response.data.token);
      return response.data;
    } catch (error) {
      this.handleError(error, 'Error authenticating user');
    }
  }

  async getProjects(): Promise<Project[]> {
    try {
      const token = localStorage.getItem('token');
      const response = await axios.get<{projects: Project[]}>(`${API_BASE_URL}/projects`, {
        headers: { Authorization: `Bearer ${token}` },
      });
      return response.data.projects;
    } catch (error) {
      this.handleError(error, 'Error fetching projects');
    }
  }

  async submitTimeEntry(projectId: string, hours: number, date: Date): Promise<TimeEntryResponse> {
    try {
      const token = localStorage.getItem('token');
      const response = await axios.post<TimeEntryResponse>(`${API_BASE_URL}/time-entries`, {
        projectId,
        hours,
        date,
      }, {
        headers: { Authorization: `Bearer ${token}` },
      });
      return response.data;
    } catch (error) {
      this.handleError(error, 'Error submitting time entry');
    }
  }

  logout(): void {
    localStorage.removeItem('token');
  }

  private handleError(error: AxiosError, message: string): never {
    console.error(message, error.message);
    throw new Error(`${message}. Cause: ${error.response?.data?.error || error.message}`);
  }
}

export const apiService = new ApiService();