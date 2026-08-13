/**
 * Icono de Bizum Voice: un micrófono.
 *
 * Ocupa el sitio del isotipo original en la navegación lateral, con el mismo
 * viewBox (126×135) y las mismas clases de color, de modo que hereda el tema
 * activo igual que el resto de iconos de la barra.
 */
const BizumVoiceMark = ({
  width,
  height,
}: {
  width?: number | string;
  height?: number | string;
}) => (
  <svg
    width={width || 126}
    height={height || 135}
    viewBox="0 0 126 135"
    className="fill-text stroke-text"
    xmlns="http://www.w3.org/2000/svg"
  >
    {/* Cápsula del micrófono */}
    <rect x="45" y="12" width="36" height="66" rx="18" stroke="none" />
    {/* Arco de sujeción */}
    <path
      d="M27 68v6a36 36 0 0 0 72 0v-6"
      fill="none"
      strokeWidth="9"
      strokeLinecap="round"
    />
    {/* Pie */}
    <path
      d="M63 110v13"
      fill="none"
      strokeWidth="9"
      strokeLinecap="round"
    />
    <path
      d="M42 123h42"
      fill="none"
      strokeWidth="9"
      strokeLinecap="round"
    />
  </svg>
);

export default BizumVoiceMark;
