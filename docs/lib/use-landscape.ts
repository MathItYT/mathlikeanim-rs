import React from "react";

const useLandscape = () => {
  const [isLandscape, setIsLandscape] = React.useState<boolean>(
    false
  );
  React.useEffect(() => {
    const handleOrientationChange = () => {
      setIsLandscape(
        window.innerWidth > window.innerHeight
      );
    };
    handleOrientationChange();
    window.addEventListener(
      "resize",
      handleOrientationChange
    );
    return () => {
      window.removeEventListener(
        "resize",
        handleOrientationChange
      );
    };
  }, []);
  return isLandscape;
};

export default useLandscape;