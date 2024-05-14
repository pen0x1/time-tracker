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
  private projectsCache?: Project[]; // Cache for storing projects
  private projectsCacheTimestamp?: number; // Timestamp for cache

  async authenticateUser(email: string, password: string): Promise<LoginResponse> {
    try {
      // Invalidate cache on new login, assuming different user might have different projects.
      this.projectsCache = undefined;
      this.projectsCacheTimestamp = undefined;

      const response = await axios.post<LoginResponse>(`${API_BASE_URL}/auth/login`, { email, password });
      localStorage.setItem('token', response.data.token);
      return response.data;
    } catch (error) {
      this.handleError(error, 'Error authenticating user');
    }
  }

  async getProjects(): Promise<Project[]> {
    const cacheDuration = 5 * 60 * 1000; // 5 minutes cache duration
    
    // Serve from cache if it's available and not expired
    if (this.projectsCache && this.projectsCacheTimestamp && (Date.now() - this.projectsCacheTimestamp) < cacheDuration) {
      return this.projectsCache;
    }

    try {
      const token = localStorage.getItem('token');
      const response = await axios.get<{projects: Project[]}>(`${API_BASE_URL}/projects`, {
        headers: { Authorization: `Bearer ${token}` },
      });

      // Update cache with new data
      this.projectsCache = response.data.projects;
      this.projectsCacheTimestamp = Date.now();

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
    // Clear cache on logout
    this.projectsCache = undefined;
    this.projectsCacheTimestamp = undefined;
  }

  private handleError(error: AxiosError, message: string): never {
    console.error(message, error.message);
    throw new Error(`${message}. Cause: ${error.response?.data?.error || error.message}`);
  }
}

export const apiService = new ApiService();