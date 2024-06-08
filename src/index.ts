import "module-alias/register";
import env from "./env";

import { feathers } from "@feathersjs/feathers";
import {
  koa,
  rest,
  bodyParser,
  errorHandler,
  serveStatic,
} from "@feathersjs/koa";
import socketio from "@feathersjs/socketio";
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

type ServiceTypes = {
  messages: MessageService;
};

const app = koa<ServiceTypes>(feathers());

// Middleware
app.use(serveStatic("./public")); //public folder as static file host
app.use(errorHandler());
app.use(bodyParser());
// Configuration
app.configure(rest()); // Register REST service handler
//register message service on Feathers application
app.use("messages", new MessageService());

app.listen(env?.PORT).then(() => {
  console.log(`Feathers server listening on localhost:${process.env.PORT}`);
});

app.service("messages").create({
  text: "Hello world from the server",
});
