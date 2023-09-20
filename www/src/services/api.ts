import type TicketModel from '../models/ticket';
import axios from 'axios';

const API_URL = 'http://127.0.0.1:8000';

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
    const response = await axios.get(`${API_URL}/ticket/${id}`);
    return response.data.ticket as TicketModel;
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
    const response = await axios.post(`${API_URL}/ticket`, {
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

export { checkStatus, getTicket, issueTicket };