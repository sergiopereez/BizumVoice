/**
 * Logotipo de Bizum Voice.
 *
 * Wordmark tipográfico en Bricolage Grotesque, la tipografía de la plataforma
 * regulatoria (empaquetada en styles/fonts.css). "bizum" en minúsculas y cian
 * de marca, con el descriptor debajo. Mantiene el viewBox del logo original
 * (930×328) para que los tamaños que ya usan Sidebar y Onboarding encajen.
 *
 * Sustituir por el SVG oficial de marca cuando esté disponible.
 */
const BizumVoiceLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <svg
      width={width}
      height={height}
      className={className}
      viewBox="0 0 930 328"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      role="img"
      aria-label="Bizum Voice"
    >
      <text
        x="0"
        y="196"
        fontFamily="'Bricolage Grotesque', system-ui, sans-serif"
        fontSize="230"
        fontWeight="700"
        letterSpacing="-8"
        className="logo-primary"
      >
        bizum
      </text>
      <text
        x="6"
        y="300"
        fontFamily="'Bricolage Grotesque', system-ui, sans-serif"
        fontSize="92"
        fontWeight="500"
        letterSpacing="30"
        className="fill-text"
      >
        VOICE
      </text>
    </svg>
  );
};

export default BizumVoiceLogo;
