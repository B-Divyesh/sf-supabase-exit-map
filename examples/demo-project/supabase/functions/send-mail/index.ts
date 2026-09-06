import { createClient } from "npm:@supabase/supabase-js";

const apiKey = Deno.env.get("MAILER_API_KEY");
const client = createClient("https://project.example.test", apiKey || "");
await fetch("https://mail.example.test/send", { method: "POST" });
void client;
