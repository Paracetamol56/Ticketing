
interface TicketModel {
  id: string;
  number: number;
  email: string;
  message: string;
  status: "open"|"pending"|"closed";
  created_at: string;
  updated_at: string|null;
  closed_at: string|null;
}

export default TicketModel;