import pg from "../lib/db";

interface Message {
  id?: number;
  text: string;
}

class MessageService {
  messages: Message[] = [];

  async find() {
    return this.messages;
  }

  async get() {
    return pg.select("*").from("anime");
  }

  async create(data: Pick<Message, "text">) {
    const message: Message = {
      id: this.messages.length,
      text: data.text,
    };
    this.messages.push(message);
    return message;
  }
}
