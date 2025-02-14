import type TicketModel from '../models/ticket';
import axios from 'axios';

const API_URL: string = "https://api.ticket.matheo-galuba.com";

async function checkStatus(): Promise<boolean> {
  try {
    const response = await axios.get(`${API_URL}/status`);
    return response.data.status === 'ok';
  } catch (error) {
    console.error(error);
    return false;
  }
}

async function getTicket(id: string): Promise<TicketModel | null> {
  try {
    const response = await axios.get(`${API_URL}/tickets/${id}`);
    return response.data as TicketModel;
  } catch (error) {
    console.error(error);
    return null;
  }
}

interface TicketPage {
  tickets: TicketModel[];
  count: number;
  max_page: number;
  page: number;
  limit: number;
  status: string | null;
}

async function getTicketPage(token: string, page: number): Promise<TicketPage | null> {
  try {
    const response = await axios.get(`${API_URL}/tickets?page=${page}&limit=10`, {
      headers: {
        'Authorization': token,
      }
    });
    return response.data as TicketPage;
  } catch (error) {
    console.error(error);
    return null;
  }
}

async function updateTicket(token: string, id: string, status: string, note: string | null, notify: boolean): Promise<TicketModel | null> {
  try {
    const response = await axios.patch(`${API_URL}/tickets/${id}`, {
      status,
      note,
      notify,
    }, {
      headers: {
        'Authorization': token,
      }
    });
    return response.data as TicketModel;
  } catch (error) {
    console.error(error);
    return null;
  }
}

async function issueTicket(
  name: string,
  email: string,
  message: string,
): Promise<TicketModel | null> {
  try {
    const response = await axios.post(`${API_URL}/tickets`, {
      name,
      email,
      message,
    });
    console.log(response);
    return response.data.ticket as TicketModel;
  } catch (error) {
    console.error(error);
    return null;
  }
}

export { checkStatus, getTicket, getTicketPage, updateTicket, issueTicket };
