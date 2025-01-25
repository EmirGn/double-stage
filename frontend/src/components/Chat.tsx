import { useParams } from "react-router";
export default function Chat() {
  const params = useParams();
  return (
    <div className="w-full h-screen p-6 text-black" data-oid="w13xr5.">
      <h1>Chat {params.id}</h1>
    </div>
  );
}
