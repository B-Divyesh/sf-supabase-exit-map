create extension if not exists pg_cron;

create table public.notes (
  id uuid primary key,
  owner_id uuid not null references auth.users(id),
  body text not null
);

alter table public.notes enable row level security;
create policy "read own notes" on public.notes
  for select using (auth.uid() = owner_id);

create or replace function public.note_updated_at()
returns trigger security definer as $$
begin
  return new;
end;
$$ language plpgsql;

create trigger note_updated before update on public.notes
  for each row execute function public.note_updated_at();

insert into storage.buckets (id, name) values ('attachments', 'attachments');
alter publication supabase_realtime add table public.notes;
select cron.schedule('archive-notes', '0 2 * * *', 'select 1');
select vault.create_secret('mailer', 'sample-only-value');
select net.http_post(url := 'https://mail.example.test/send');
