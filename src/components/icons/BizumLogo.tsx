/**
 * Isotipo de Bizum.
 *
 * Es el mismo símbolo que usa la plataforma regulatoria en su favicon,
 * reconstruido como vector para que no pierda nitidez al escalar: cinco piezas
 * (dos puntos y tres cápsulas) sobre la misma diagonal 3:4, con grosor de trazo
 * uniforme. La geometría se midió sobre el original y coincide con él.
 *
 * Hereda el color del texto (`currentColor`), de modo que funciona igual sobre
 * el tema claro y el oscuro.
 */
const BizumLogo = ({
  size,
  className,
}: {
  size?: number;
  className?: string;
}) => {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 96 96"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
      role="img"
      aria-label="Bizum"
    >
      <circle cx="29" cy="20" r="5.6" fill="currentColor" />
      <path
        d="M39.63 33.13L45.17 25.73"
        stroke="currentColor"
        strokeWidth="11.2"
        strokeLinecap="round"
      />
      <path
        d="M36.47 65.07L61.89 31.19"
        stroke="currentColor"
        strokeWidth="11.2"
        strokeLinecap="round"
      />
      <path
        d="M53.14 70.43L59.08 62.51"
        stroke="currentColor"
        strokeWidth="11.2"
        strokeLinecap="round"
      />
      <circle cx="69.5" cy="76.31" r="5.6" fill="currentColor" />
    </svg>
  );
};

export default BizumLogo;
